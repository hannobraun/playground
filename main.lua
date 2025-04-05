local field_width = 10
local field_height = 16

local block_size = 32
local block_offset = 1
local block_size_inner = block_size - 2 * block_offset

function love.draw()
    for i = 0, field_width - 1, 1 do
        for j = 1, field_height, 1 do
            local x = i * block_size
            local y = block_size * field_height - j * block_size
            
            love.graphics.rectangle(
                "fill",
                x, y,
                block_size_inner, block_size_inner
            )
        end
    end
end
