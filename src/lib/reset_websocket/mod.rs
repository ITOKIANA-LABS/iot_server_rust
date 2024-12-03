pub fn reset_websocket() {
    let command_reset = "./restart_1.sh";
    let args_reset = None;


    match crate::lib::cmd_linux::function(command_reset, args_reset) {
        Ok(result) => {
          println!("Result: {}", result);
            let _ = result.trim().to_string();
        }
        Err(error) => {
            println!("Error: {}", error);
        }
    }
}