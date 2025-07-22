use crate::compiler::parser::{Expression, Program};

pub fn compile_program(program: Program) -> anyhow::Result<Vec<u8>> {
    let number_of_functions = program.number_of_functions()?.into();

    let functions = (0..)
        .zip(program.functions)
        .map(|(index, function)| Function {
            index,
            name: function.name.as_bytes().to_vec(),
            body: function.body,
        })
        .collect::<Vec<_>>();

    let mut code = Vec::new();

    let magic = b"\0asm";
    code.extend(magic);

    let version = [1, 0, 0, 0];
    code.extend(version);

    {
        let type_section = 1;

        let data = {
            let mut data = Vec::new();
            leb128::write::unsigned(&mut data, number_of_functions)?;
            generate_function_type(&mut data)?;

            data
        };

        let Ok(size) = data.len().try_into() else {
            anyhow::bail!("Type section length doesn't fit into `u32`.");
        };
        let size: u32 = size;
        let size: u64 = size.into();

        code.extend([type_section]);
        leb128::write::unsigned(&mut code, size)?;
        code.extend(data);
    }

    {
        let function_section = 3;

        let data = {
            let mut data = Vec::new();
            leb128::write::unsigned(&mut data, number_of_functions)?;
            for function in &functions {
                generate_function(function, &mut data)?;
            }

            data
        };

        let Ok(size) = data.len().try_into() else {
            anyhow::bail!("Function section length doesn't fit into `u32`.");
        };
        let size: u32 = size;
        let size: u64 = size.into();

        code.extend([function_section]);
        leb128::write::unsigned(&mut code, size)?;
        code.extend(data);
    }

    {
        let export_section = 7;

        let data = {
            let mut data = Vec::new();
            leb128::write::unsigned(&mut data, number_of_functions)?;
            for function in &functions {
                generate_function_export(function, &mut data)?;
            }

            data
        };

        let Ok(size) = data.len().try_into() else {
            anyhow::bail!("Export section length doesn't fit into `u32`.");
        };
        let size: u32 = size;
        let size: u64 = size.into();

        code.extend([export_section]);
        leb128::write::unsigned(&mut code, size)?;
        code.extend(data);
    }

    {
        let code_section = 10;

        let data = {
            let mut data = Vec::new();
            leb128::write::unsigned(&mut data, number_of_functions)?;
            for function in &functions {
                compile_function_body(&function.body, &mut data)?;
            }

            data
        };

        let Ok(size) = data.len().try_into() else {
            anyhow::bail!("Code section length doesn't fit into `u32`.");
        };
        let size: u32 = size;
        let size: u64 = size.into();

        code.extend([code_section]);
        leb128::write::unsigned(&mut code, size)?;
        code.extend(data);
    }

    Ok(code)
}

fn generate_function_type(output: &mut Vec<u8>) -> anyhow::Result<()> {
    let function_type = 0x60;
    let number_of_parameters = 0;
    let number_of_results = 1;
    let type_i32 = 0x7f;

    output.extend([function_type]);
    leb128::write::unsigned(output, number_of_parameters)?;
    leb128::write::unsigned(output, number_of_results)?;
    output.extend([type_i32]);

    Ok(())
}

fn generate_function(
    function: &Function,
    output: &mut Vec<u8>,
) -> anyhow::Result<()> {
    let type_index = function.index.into();
    leb128::write::unsigned(output, type_index)?;

    Ok(())
}

fn generate_function_export(
    function: &Function,
    output: &mut Vec<u8>,
) -> anyhow::Result<()> {
    let Ok(size_of_name) = function.name.len().try_into() else {
        anyhow::bail!("Function section length doesn't fit into `u32`.");
    };
    let size_of_name: u32 = size_of_name;
    let size_of_name: u64 = size_of_name.into();

    let function_index = 0x00;
    let index_of_function = 0;

    leb128::write::unsigned(output, size_of_name)?;
    output.extend(&function.name);
    output.extend([function_index]);
    leb128::write::unsigned(output, index_of_function)?;

    Ok(())
}

fn compile_function_body(
    body: &Expression,
    output: &mut Vec<u8>,
) -> anyhow::Result<()> {
    let code = {
        let number_of_locals = 0;
        let end = 0x0b;

        let mut code = Vec::new();
        leb128::write::unsigned(&mut code, number_of_locals)?;
        compile_expression(body, &mut code)?;
        code.extend([end]);

        code
    };

    let Ok(size) = code.len().try_into() else {
        anyhow::bail!("Code section length doesn't fit into `u32`.");
    };
    let size: u32 = size;
    let size: u64 = size.into();

    leb128::write::unsigned(output, size)?;
    output.extend(code);

    Ok(())
}

fn compile_expression(
    expression: &Expression,
    output: &mut Vec<u8>,
) -> anyhow::Result<()> {
    let instruction_i32 = 0x41;

    let Expression::Literal { value } = expression;
    let n = (*value).into();

    output.extend([instruction_i32]);
    leb128::write::signed(output, n)?;

    Ok(())
}

pub struct Function {
    pub index: u32,
    pub name: Vec<u8>,
    pub body: Expression,
}
