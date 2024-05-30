use rustpython_vm as vm;
use rustpython_vm::function::PosArgs;
use std::env;
use std::process::ExitCode;
use vm::{builtins::PyStrRef, Interpreter};

fn py_main(interp: &Interpreter) -> vm::PyResult<PyStrRef> {
    interp.enter(|vm| {
        // Add local library path
        // vm.insert_sys_path(vm.new_pyobj("examples"))
        // .expect("add examples to sys.path failed, why?");

        // Import the requests library
        let module = vm.import("idf_tools", 0)?;

        let name_func = module.get_attr("action_install", vm)?;
        // params setup
        let quiet = vm.ctx.false_value.clone();
        let non_interactive = vm.ctx.new_bool(false);
        let tools_json = vm.ctx.new_str("./examples/tools.json");
        let idf_path = vm.ctx.none();
        let tools = vm.ctx.new_list(vec![vm.ctx.new_str("all").into()]);
        let targets = vm.ctx.new_str("all");

        let pos_args: PosArgs = PosArgs::new(vec![
            quiet.into(),
            non_interactive.into(),
            tools_json.into(),
            idf_path,
            tools.into(),
            targets.into(),
        ]);

        let result = name_func.call(pos_args, vm)?;
        let result_str = result.str(vm)?;
        let result_pystrref: PyStrRef = result_str.into();
        // let result: PyStrRef = result.get_attr("name", vm)?.try_into_value(vm)?;
        vm::PyResult::Ok(result_pystrref)
    })
}

fn main() -> ExitCode {
    // Add standard library path
    let mut settings = vm::Settings::default();
    settings.path_list.push("Lib".to_owned());
    match env::var("RUSTPYTHONPATH") {
        Ok(path) => {
            settings
                .path_list
                .extend(path.split(':').map(|s| s.to_owned()));
        }
        Err(_) => {}
    }
    let interp = vm::Interpreter::with_init(settings, |vm| {
        vm.add_native_modules(rustpython_stdlib::get_module_inits());
    });

    let result = py_main(&interp);
    let result = result.map(|result| {
        println!("Result: {result}");
    });
    ExitCode::from(interp.run(|_vm| result))
}
