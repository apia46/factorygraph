extends Control

var themeDefault : Theme
var themeSelected : Theme
var gui : PanelContainer
var graph : Node2D

var selected = false
var collapsed = false

var moving = false
var preMoveDiff : Vector2

func init(Gui:PanelContainer, Graph:Node2D, Rect:Rect2, ThemeDefault:=preload("res://templates/themes/componentDefault.tres"), ThemeSelected:=preload("res://templates/themes/componentSelected.tres")) -> Control:
	gui = Gui
	graph = Graph
	themeDefault = ThemeDefault
	themeSelected = ThemeSelected
	position = Rect.position
	%container.size = Rect.size
	set_theme(themeDefault)
	add_to_group("components")
	return self

func _process(_delta):
	if moving: position = preMoveDiff + graph.viewport.get_mouse_position()

func gui_input(event):
	if event is InputEventMouseButton:
		click(event)

func click(event:InputEvent) -> void:
	match gui.editMode:
		gui.SELECT:
			if !event.pressed:
				if selected: unselect()
				else: select()
				get_viewport().set_input_as_handled()

func selectInRect(rect:Rect2) -> void:
	if rect.intersects(Rect2(position + %iconContainer.position, %iconContainer.size)) or (!collapsed and rect.intersects(Rect2(position, %container.size))): select()

func select() -> void: if !selected:
	set_theme(themeSelected); selected = true; graph.selecteds += 1
func unselect() -> void: if selected:
	set_theme(themeDefault); selected = false; graph.selecteds -= 1

func startMove() -> void: if selected or graph.selecteds == 0: preMoveDiff = position - graph.viewport.get_mouse_position(); moving = true
func stopMove() -> void: moving = false

func _icon_pressed():
	collapsed = !collapsed
	%container.visible = !collapsed
