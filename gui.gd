extends PanelContainer

enum {SELECT, MOVE, ADD_COMPONENT}
var editMode = SELECT

func _input(_event):
	if Input.is_action_just_pressed("editMode_select"):
		changeTab(SELECT)
	if Input.is_action_just_pressed("editMode_move"):
		changeTab(MOVE)
	if Input.is_action_just_pressed("editMode_addComponent"):
		changeTab(ADD_COMPONENT)

func changeTab(tab : int) -> void:
	%editMode.current_tab = tab
	_edit_mode_changed(tab)

func _edit_mode_changed(tab):
	editMode = tab
