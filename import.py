import json, math, mathutils, bpy, time
start = time.time()
j = json.load(open("/home/nocrex/Documents/Nextcloud/Programming/blender-demo-viewer/extractor/list.json"))
time_json = time.time() - start

bpy.context.scene.render.fps = 200
bpy.context.scene.render.fps_base = 3
bpy.context.scene.frame_end = 23908
bpy.ops.object.empty_add(type='ARROWS', align='WORLD', location=(0, 0, 0), scale=(1, 1, 1))

empty = bpy.context.scene.objects["Empty"]

tick_start = time.time()
for t in j:
    tick = t[0]
    angles = t[1][0]['angles']
    location = t[1][0]['origin']
    
    empty.rotation_euler = mathutils.Euler((math.radians(angles['z']), math.radians(angles['x']), math.radians(angles['y'])), 'XYZ')
    empty.keyframe_insert(data_path="rotation_euler", frame=tick)
    
    empty.location = mathutils.Vector(location.values()) / 16 / 3.28084
    empty.keyframe_insert(data_path="location", frame=tick)

print(f"{time.time()-start} seconds, {time_json} json parsing, {time.time()-tick_start} creating keyframes")