# MineCraft-clone-

minecraft clone no godot usando rust

minecraft clone in godot using rust


?: como q faz pra pegar um node
unsafe {
	owner.get_node(NodePath).unwrap().assume_safe().cast::<NodeType>().unwrap()
}
?: como juntar/transformar duas (ou mais) strings/int/float/etc em uma so string
format!()
NOTE:apenas use "st.add_smooth_group(true);" qnd for fazer voxels regulares
NOTE:o godot crasha qnd usa "#[register_with(Self::register_properties)]"


?: how to get a node
unsafe {
	owner.get_node(NodePath).unwrap().assume_safe().cast::<NodeType>().unwrap()
}
?: how to join/transform two (or more) strings/int/float/etc in one string
format!()
NOTE:only use "st.add_smooth_group(true);" when making smooth voxels
NOTE:godot crash when using "#[register_with(Self::register_properties)]"

