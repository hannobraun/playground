local Field = {
    border = 1,
    margin = 1,
}

function Field:offset()
    return self.border + self.margin
end


local Cell = {
    size = 32,
}


local Block = {
    size = 30
}

function Block:offset()
    return (Cell.size - self.size) / 2
end


local function draw_border(field)
    local border_size_x = field.size_x * Cell.size + Field:offset() * 2
    local border_size_y = field.size_y * Cell.size + Field:offset() * 2

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
        Field.border, Field.border,
        border_size_x - Field.border * 2, border_size_y - Field.border * 2
    )
    love.graphics.setColor(fg_r, fg_g, fg_b, fg_a)
end

local function draw(field)
    draw_border(field)

    for x = 0, field.size_x - 1 do
        for y = 0, field.size_y - 1 do
            if field[field.index(x, y)] then
                love.graphics.rectangle(
                    "fill",
                    Field:offset() + x * Cell.size + Block:offset(),
                    Field:offset() + y * Cell.size + Block:offset(),
                    Block.size, Block.size
                )
            end
        end
    end
end


return {
    draw = draw,
}
