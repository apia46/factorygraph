extends Node2D
const COMPONENT = preload("res://component.tscn")
const RECTANGLESELECT = preload("res://rectangleSelect.tscn")

@onready var gui = get_node("/root/gui")
@onready var viewport = get_node("/root/gui/margins/vBox/graphCont/graphViewport")

var components = []
var selecteds = 0
var select : Node2D

func _input(event):
	if event is InputEventMouseButton:
		click(event)

func _unhandled_input(event):
	if event is InputEventMouseButton:
		match gui.editMode:
			gui.SELECT:
				if event.pressed:
					select = RECTANGLESELECT.instantiate().init("Select", viewport.get_mouse_position() - Vector2(16,16), viewport)
					add_child(select)
					get_viewport().set_input_as_handled()
				else:
					var rect = select.getSelected()
					if rect.size < Vector2(4,4):
						get_tree().call_group("components", "unselect")
					get_tree().call_group("components", "selectInRect", rect)
					select.queue_free()
					select = null
					get_viewport().set_input_as_handled()

func click(event:InputEvent) -> void:
	match gui.editMode:
		gui.MOVE:
			if event.pressed:
				get_tree().call_group("components", "startMove")
				get_viewport().set_input_as_handled()
			else:
				get_tree().call_group("components", "stopMove")
				get_viewport().set_input_as_handled()
		gui.ADD_COMPONENT:
			if event.pressed:
				select = RECTANGLESELECT.instantiate().init("Add", viewport.get_mouse_position() - Vector2(16,16), viewport, false, Vector2(64,64), true)
				add_child(select)
				get_viewport().set_input_as_handled()
			if !event.pressed:
				var newComponent = COMPONENT.instantiate().init(gui, self, select.getSelected(), )
				select.queue_free()
				select = null
				components.append(newComponent)
				add_child(newComponent)
				if !Input.is_action_pressed("shift"): gui.changeTab(gui.SELECT)
				get_viewport().set_input_as_handled()
