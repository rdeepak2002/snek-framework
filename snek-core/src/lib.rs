use gc::{Finalize, Trace};
use rustpython_vm::builtins::{PyDictRef, PyIntRef};
use rustpython_vm::convert::{ToPyObject, ToPyResult};
use rustpython_vm::function::{ArgMapping, FuncArgs, IntoFuncArgs, OptionalArg};
use rustpython_vm::protocol::PyNumber;
use rustpython_vm::scope::Scope;
use rustpython_vm::{
    compiler, pyclass, pymodule,
    types::{Constructor, GetDescriptor, Unconstructible},
    vm, AsObject, Interpreter, PyObject, PyObjectRef, PyPayload, PyResult, TryFromBorrowedObject,
    VirtualMachine,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn init_logger() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
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
            website_code = String::from("<div style=\"color: red;\">Error</div>");
            return;
        }
        let code_obj = code_obj.unwrap();
        let res = vm.run_code_obj(code_obj, scope).map(|value| {
            if let Ok(render) = value.get_attr("render", vm) {
                let args = vec![];
                let res = render.call(args, vm);
                if let Err(..) = res {
                    let mut error_content = String::new();
                    let e = res.as_ref().unwrap_err();
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
                    website_code =
                        String::from(format!("<div style=\"color: red;\">{error_content}</div>"));
                } else {
                    website_code = String::from(res.as_ref().unwrap().str(vm).unwrap().as_str());
                    log::debug!("{}", website_code);
                }
            }
        });
        if res.is_err() {
            website_code =
                String::from(format!("<div style=\"color: red;\">Fatal exception</div>"));
        }
    });
    String::from(website_code)
}

#[pymodule]
pub(crate) mod snek_core {
    use std::any::Any;

    use rustpython_vm::builtins::{PyGenericAlias, PyStr, PyStrInterned, PyTypeRef};
    use rustpython_vm::protocol::PyNumberMethods;
    use rustpython_vm::types::AsNumber;
    use rustpython_vm::{
        builtins::PyList, convert::ToPyObject, Py, PyObjectRef, TryFromBorrowedObject,
    };

    use super::*;

    #[pyfunction]
    fn add_two_internal(num: u64, _vm: &VirtualMachine) -> PyResult<u64> {
        Ok(num + 2)
    }
}
