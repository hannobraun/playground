local lick = require "lick"

function love.draw()
    local field_size_x = 10
    local field_size_y = 16

    local cell_size = 32

    local block_size = 30
    local block_offset = (cell_size - block_size) / 2

    love.graphics.rectangle(
        "line",
        0, 0,
        field_size_x * cell_size, field_size_y * cell_size        
    )

    for i = 0, field_size_x - 1 do
        for j = 0, field_size_y - 1 do
            love.graphics.rectangle(
                "fill",
                i * cell_size + block_offset, j * cell_size + block_offset,
                block_size, block_size
            )
        end
    end
end
