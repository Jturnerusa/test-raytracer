import bpy, itertools, json, bmesh, sys, math
from dataclasses import dataclass
from more_itertools import flatten
from bpy_extras import io_utils


def export_light(light, view):
    match light.type:
        case "AREA":
            return {
                "Area": {
                    "size": light.size,
                    "transform": list(flatten(map(lambda x: list(x), list(view)))),
                    "power": light.energy,
                    "color": [light.color[0], light.color[1], light.color[2], 0],
                }
            }
        case _:
            raise Exception("unsupported light type")


def export_material(material):
    if material.metallic > 0:
        return {
            "Metal": {
                "color": list(material.diffuse_color),
                "roughness": material.roughness,
            }
        }
    else:
        return {
            "Diffuse": {
                "color": list(material.diffuse_color),
                "adsorption": material.roughness,
            }
        }


def export_camera(camera, view):
    return {
        "fov": math.radians(90),
        "aspect_ratio": 1.77,
        "znear": camera.clip_start,
        "zfar": camera.clip_end,
        "transform": list(flatten(map(lambda x: list(x), list(view)))),
    }


def export_mesh(mesh):
    bm = bmesh.new()
    bm.from_mesh(mesh)

    verts = list(map(lambda v: list(v.co), bm.verts))
    normals = list(map(lambda v: list(v.normal), bm.verts))

    faces = []

    for face in bmesh.ops.triangulate(bm, faces=bm.faces)["faces"]:
        faces.append(list(map(lambda v: v.index, face.verts)))

    return {"verts": verts, "normals": normals, "faces": faces}


def export_object(object):
    mesh = object.data.name
    transform = list(flatten(map(lambda x: list(x), list(object.matrix_world))))
    material = object.material_slots[0].name

    return {"mesh": mesh, "transform": transform, "material": material}


meshes = {}
materials = {}
objects = []
lights = []

for mesh in bpy.data.meshes:
    meshes[mesh.name] = export_mesh(mesh)

for material in bpy.data.materials:
    materials[material.name] = export_material(material)

for obj in bpy.context.scene.objects:
    match obj.type:
        case "MESH":
            objects.append(export_object(obj))
        case "LIGHT":
            light = obj.data
            lights.append(export_light(light, obj.matrix_world))

camera = export_camera(
    bpy.context.scene.objects["Camera"].data,
    bpy.context.scene.objects["Camera"].matrix_world,
)

outfile = sys.argv[sys.argv.index("--") + 1]

with open(outfile, "w") as output:
    print(
        json.dumps(
            {
                "background": [0, 0, 0, 0],
                "meshes": meshes,
                "objects": objects,
                "lights": lights,
                "materials": materials,
                "camera": camera,
            }
        ),
        file=output,
    )


# Local Variables:
# fmt-executable: "black"
# fmt-args: ("-")
# eval: (add-hook 'before-save-hook 'fmt-current-buffer nil t)
# End:
