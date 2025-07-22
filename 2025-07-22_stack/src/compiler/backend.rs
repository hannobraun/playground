pub fn generate() -> anyhow::Result<Vec<u8>> {
    let mut module = Vec::new();

    let magic = b"\0asm";
    module.extend(magic);

    let version = [1, 0, 0, 0];
    module.extend(version);

    {
        let type_section = 1;

        let data = {
            let number_of_types = 1;
            let function_type = 0x60;
            let number_of_parameters = 0;
            let number_of_results = 1;
            let type_i32 = 0x7f;

            let mut data = Vec::new();
            leb128::write::unsigned(&mut data, number_of_types)?;
            data.extend([function_type]);
            leb128::write::unsigned(&mut data, number_of_parameters)?;
            leb128::write::unsigned(&mut data, number_of_results)?;
            data.extend([type_i32]);

            data
        };

        let Ok(size) = data.len().try_into() else {
            anyhow::bail!("Type section length doesn't fit into `u32`.");
        };
        let size: u32 = size;
        let size: u64 = size.into();

        module.extend([type_section]);
        leb128::write::unsigned(&mut module, size)?;
        module.extend(data);
    }

    {
        let function_section = 3;

        let data = {
            let number_of_functions = 1;
            let type_index = 0;

            let mut data = Vec::new();
            leb128::write::unsigned(&mut data, number_of_functions)?;
            leb128::write::unsigned(&mut data, type_index)?;

            data
        };

        let Ok(size) = data.len().try_into() else {
            anyhow::bail!("Function section length doesn't fit into `u32`.");
        };
        let size: u32 = size;
        let size: u64 = size.into();

        module.extend([function_section]);
        leb128::write::unsigned(&mut module, size)?;
        module.extend(data);
    }

    {
        let export_section = 7;

        let data = {
            let number_of_exports = 1;

            let name = b"start";

            let Ok(size_of_name) = name.len().try_into() else {
                anyhow::bail!(
                    "Function section length doesn't fit into `u32`."
                );
            };
            let size_of_name: u32 = size_of_name;
            let size_of_name: u64 = size_of_name.into();

            let function_index = 0x00;
            let index_of_function = 0;

            let mut data = Vec::new();
            leb128::write::unsigned(&mut data, number_of_exports)?;
            leb128::write::unsigned(&mut data, size_of_name)?;
            data.extend(name);
            data.extend([function_index]);
            leb128::write::unsigned(&mut data, index_of_function)?;

            data
        };

        let Ok(size) = data.len().try_into() else {
            anyhow::bail!("Export section length doesn't fit into `u32`.");
        };
        let size: u32 = size;
        let size: u64 = size.into();

        module.extend([export_section]);
        leb128::write::unsigned(&mut module, size)?;
        module.extend(data);
    }

    {
        let code_section = 10;
        let number_of_functions = 1;

        let data = {
            let code = {
                let number_of_locals = 0;
                let instruction_i32 = 0x41;
                let n = 42;
                let end = 0x0b;

                let mut code = Vec::new();
                leb128::write::unsigned(&mut code, number_of_locals)?;
                code.extend([instruction_i32]);
                leb128::write::signed(&mut code, n)?;
                code.extend([end]);

                code
            };

            let Ok(size) = code.len().try_into() else {
                anyhow::bail!("Code section length doesn't fit into `u32`.");
            };
            let size: u32 = size;
            let size: u64 = size.into();

            let mut data = Vec::new();
            leb128::write::unsigned(&mut data, number_of_functions)?;
            leb128::write::unsigned(&mut data, size)?;
            data.extend(code);

            data
        };

        let Ok(size) = data.len().try_into() else {
            anyhow::bail!("Code section length doesn't fit into `u32`.");
        };
        let size: u32 = size;
        let size: u64 = size.into();

        module.extend([code_section]);
        leb128::write::unsigned(&mut module, size)?;
        module.extend(data);
    }

    Ok(module)
}
