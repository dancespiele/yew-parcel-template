import os


fn main() {
	build_type := if os.args.len >= 2 {
		os.args[1]
	} else {
		""
	}

	code := os.system('wasm-pack build $build_type --target web crate')
	
	if code != 0 {
		println('error in wasm-pack execution')
	} 
}