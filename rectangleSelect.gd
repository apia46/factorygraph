extends Node2D
var viewport : Viewport
var followMouse = true
var origin : Vector2
var size : Vector2
var invisible : bool
var minimum : Vector2
var clamps : bool

func init(text:String, Origin:Vector2, VIewport:Viewport, Invisible:=false, Minimum:=Vector2(4,4), Clamps:=false) -> Node2D:
	%label.text = text
	origin = Origin
	viewport = VIewport
	invisible = Invisible
	minimum = Minimum
	clamps = Clamps
	return self

func _process(_delta):
	if followMouse:
		size = viewport.get_mouse_position() - origin - Vector2(16,16)
		if clamps:
			if abs(size.x) < minimum.x: size.x = (sign(size.x) if size.x != 0 else 1) * minimum.x
			if abs(size.y) < minimum.y: size.y = (sign(size.y) if size.y != 0 else 1) * minimum.y
		position = origin + Vector2(min(0, size.x), min(0, size.y))
		%panel.custom_minimum_size = abs(size)
	visible = !invisible and (abs(size) > minimum or clamps)

func stopFollowMouse() -> void:
	followMouse = false

func getSelected() -> Rect2:
	return Rect2(position, abs(size))
