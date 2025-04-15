local lick = require "lick"

local field_size_x = 10
local field_size_y = 16

local cell_size = 32

function love.draw()
    local block_size = 30
    local block_offset = (cell_size - block_size) / 2

    draw_border()

    for i = 0, field_size_x - 1 do
        for j = 0, field_size_y - 1 do
            love.graphics.rectangle(
                "fill",
                i * cell_size + block_offset,
                j * cell_size + block_offset,
                block_size, block_size
            )
        end
    end
end

function draw_border()
    local border_size_x = field_size_x * cell_size
    local border_size_y = field_size_y * cell_size

    local bg_r, bg_g, bg_b, bg_a = love.graphics.getBackgroundColor()
    local fg_r, fg_g, fg_b, fg_a = love.graphics.getColor()

    local border_strength = 1

    love.graphics.rectangle(
        "fill",
        0, 0,
        border_size_x, border_size_y
    )

    love.graphics.setColor(bg_r, bg_g, bg_b, bg_a)
    love.graphics.rectangle(
        "fill",
        border_strength, border_strength,
        border_size_x - border_strength * 2, border_size_y - border_strength * 2
    )
    love.graphics.setColor(fg_r, fg_g, fg_b, fg_a)
end