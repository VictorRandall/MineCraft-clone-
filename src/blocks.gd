extends Spatial

const chunk_size = 16
const chunk_amount = 15


var noise_seed:int
var chunks:Dictionary = {}
var unready_chunks:Dictionary = {}
var thread:Thread

func _ready() -> void:
	randomize()
	noise_seed = randi()
	thread = Thread.new()
	
	
func add_chunk(x, z):
	var key = str(x) + "," + str(z)
	
	# check if chunk already exists or if it is in the unready_chunks (going to exist)
	if chunks.has(key) or unready_chunks.has(key):
		return
		
	# see if thread is not active at the moment (working on another chunk)
	if not thread.is_active():
		thread.start(self, "load_chunk", [thread, x, z])
		unready_chunks[key] = 1
		
func load_chunk(arr):
	
	var thread = arr[0]
	var x = arr[1]
	var z = arr[2]

	# generate chunk
	var chunk = VoxelChunk.new(noise_seed, x * chunk_size, z * chunk_size, chunk_size)
	
	# set chunk position
	chunk.translation = Vector3(x * chunk_size, 0, z * chunk_size)
	
	
	call_deferred("load_done", chunk, thread)
	

func load_done(chunk, thread):
	add_child(chunk)
	var key = str(chunk.x / chunk_size) + "," + str(chunk.z / chunk_size)
	chunks[key] = chunk
	unready_chunks.erase(key)
	thread.wait_to_finish()
	
func get_chunk(x, z):
	var key = str(x) + "," + str(z)
	if chunks.has(key):
		return chunks.get(key)
		
	return null

func _process(delta):
#	update_chunks()
	var player_translation = get_node("../KinematicBody").translation
	var p_x = int(player_translation.x) / chunk_size
	var p_y = int(player_translation.y) / chunk_size
	var p_z = int(player_translation.z) / chunk_size
	
	for x in range(p_x - chunk_amount * 0.5, p_x + chunk_amount * 0.5):
		for z in range(p_z - chunk_amount * 0.5, p_z + chunk_amount * 0.5):
			add_chunk(x, z)
			var chunk = get_chunk(x, z)
			if chunk != null:
				chunk.should_remove = false
#	clean_up_chunks()
	for key in chunks:
		var chunk = chunks[key]
		if chunk.should_remove:
			chunk.queue_free()
			chunks.erase(key)
#	reset_chunks()
	for key in chunks:
		chunks[key].should_remove = true
#	pass
	
#func update_chunks():
#
#	var player_translation = $Position3D.translation
#	var p_x = int(player_translation.x) / chunk_size
#	var p_y = int(player_translation.y) / chunk_size
#	var p_z = int(player_translation.z) / chunk_size
#
#	for x in range(p_x - chunk_amount * 0.5, p_x + chunk_amount * 0.5):
#		for z in range(p_z - chunk_amount * 0.5, p_z + chunk_amount * 0.5):
#			add_chunk(x, z)
#			var chunk = get_chunk(x, z)
#			if chunk != null:
#				chunk.should_remove = false
#
#func clean_up_chunks():
#	for key in chunks:
#		var chunk = chunks[key]
#		if chunk.should_remove:
#			chunk.queue_free()
#			chunks.erase(key)
#
#func reset_chunks():
#	for key in chunks:
#		chunks[key].should_remove = true
