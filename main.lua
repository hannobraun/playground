local field_width = 10
local field_height = 16

local block_size = 32
local block_offset = 1
local block_size_inner = block_size - 2 * block_offset

local field = {}

function love.load()
    field = empty_field()
    field[5][1] = true
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
            local y = (j - 1) * block_size
            
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

function empty_field()
    local empty_field = {}

    for i = 1, field_width, 1 do
        empty_field[i] = {}

        for j = 1, field_height, 1 do
            empty_field[i][j] = false
        end
    end

    return empty_field
end