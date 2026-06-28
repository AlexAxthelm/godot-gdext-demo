class_name Palette
extends RefCounted

## Maps level-data color names to actual Color values.
## Central place to tweak the minimal flat-color look.

const COLORS := {
	"red": Color("e74c3c"),
	"blue": Color("3498db"),
	"green": Color("2ecc71"),
	"yellow": Color("f1c40f"),
	"purple": Color("9b59b6"),
	"orange": Color("e67e22"),
}

static func get_color(name: String) -> Color:
	return COLORS.get(name, Color.WHITE)
