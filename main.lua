local field_width = 10
local field_height = 16

local block_size = 32
local block_offset = 1
local block_size_inner = block_size - 2 * block_offset

local field = {}

function love.load()
    init_empty_field()
    field[5][field_height] = true
end

function love.keypressed(key)
    if key == "escape" then
        love.event.quit()
    end
end

function love.draw()
    for i = 1, field_width, 1 do
        for j = 1, field_height, 1 do
            local x = (i - 1) * block_size
            local y = block_size * field_height - j * block_size
            
            if field[i][j] then
                love.graphics.rectangle(
                    "fill",
                    x, y,
                    block_size_inner, block_size_inner
                )
            end
        end
    end
end

function init_empty_field()
    for i = 1, field_width, 1 do
        field[i] = {}

        for j = 1, field_height, 1 do
            field[i][j] = false
        end
    end
end