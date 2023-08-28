use rustpython_vm::builtins::PyBaseExceptionRef;
use rustpython_vm::{compiler, pymodule, Interpreter, PyResult, VirtualMachine};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn init_logger() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
}

pub fn get_error_content(e: &PyBaseExceptionRef, vm: &VirtualMachine) -> String {
    let mut error_content = String::new();
    let py_err_arg = e.get_arg(0);
    if py_err_arg.is_some() {
        let py_err_str = py_err_arg.unwrap().str(vm);
        if py_err_str.is_ok() {
            let py_err = py_err_str.unwrap();
            log::error!("{}", py_err);
            error_content += &*String::from(format!("{}", py_err));
        }
    }
    let traceback = e.traceback();
    if traceback.is_some() {
        let line_number = traceback.unwrap().lineno;
        log::error!("line {}", line_number);
        error_content += &*String::from(format!(", line {}", line_number));
    }
    error_content
}

#[wasm_bindgen]
pub fn python_to_html(code: &str) -> String {
    log::debug!("Executing Python code {}", code);
    let mut website_code = String::from(
        r#"
        <div>
            <h1>Header text</h1>
            <p>Paragraph text</p>
        </div>
    "#,
    );
    let interpreter = Interpreter::with_init(Default::default(), |vm| {
        vm.add_native_module("snek_core".to_owned(), Box::new(snek_core::make_module));
        vm.add_frozen(rustpython_vm::py_freeze!(dir = "src/pylib"));
    });
    interpreter.enter(|vm| {
        let scope = vm.new_scope_with_builtins();
        let source_path = "<wasm>";
        let code_obj = vm
            .compile(code, compiler::Mode::BlockExpr, source_path.to_owned())
            .map_err(|err| vm.new_syntax_error(&err, None));
        if code_obj.is_err() {
            let e = code_obj.as_ref().err().unwrap();
            let error_content = get_error_content(e, vm);
            website_code =
                String::from(format!("<div style=\"color: red;\">{error_content}</div>"));
            return;
        }
        let code_obj = code_obj.unwrap();
        let res = vm.run_code_obj(code_obj, scope).map(|value| {
            if let Ok(render) = value.get_attr("render", vm) {
                let args = vec![];
                let res = render.call(args, vm);
                if let Err(..) = res {
                    let e = res.as_ref().unwrap_err();
                    let error_content = get_error_content(e, vm);
                    website_code =
                        String::from(format!("<div style=\"color: red;\">{error_content}</div>"));
                    return;
                } else {
                    website_code = String::from(res.as_ref().unwrap().str(vm).unwrap().as_str());
                    return;
                }
            }
        });
        if res.is_err() {
            let e = res.as_ref().unwrap_err();
            let error_content = get_error_content(e, vm);
            website_code =
                String::from(format!("<div style=\"color: red;\">{error_content}</div>"));
            return;
        }
    });
    log::debug!("{}", website_code);
    String::from(website_code)
}

#[pymodule]
pub(crate) mod snek_core {
    use super::*;

    #[pyfunction]
    fn add_two_internal(num: u64, _vm: &VirtualMachine) -> PyResult<u64> {
        Ok(num + 2)
    }
}
