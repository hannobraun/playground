local lick = require "lick"

local field = {
    size_x = 10,
    size_y = 16,
}

local field_border = 1
local field_margin = 1
local field_offset = field_border + field_margin

local cell_size = 32

function love.draw()
    local block_size = 30
    local block_offset = (cell_size - block_size) / 2

    draw_border()

    for i = 0, field.size_x - 1 do
        for j = 0, field.size_y - 1 do
            love.graphics.rectangle(
                "fill",
                field_offset + i * cell_size + block_offset,
                field_offset + j * cell_size + block_offset,
                block_size, block_size
            )
        end
    end
end

function draw_border()
    local border_size_x = field.size_x * cell_size + field_offset * 2
    local border_size_y = field.size_y * cell_size + field_offset * 2

    local bg_r, bg_g, bg_b, bg_a = love.graphics.getBackgroundColor()
    local fg_r, fg_g, fg_b, fg_a = love.graphics.getColor()

    love.graphics.rectangle(
        "fill",
        0, 0,
        border_size_x, border_size_y
    )

    love.graphics.setColor(bg_r, bg_g, bg_b, bg_a)
    love.graphics.rectangle(
        "fill",
        field_border, field_border,
        border_size_x - field_border * 2, border_size_y - field_border * 2
    )
    love.graphics.setColor(fg_r, fg_g, fg_b, fg_a)
end