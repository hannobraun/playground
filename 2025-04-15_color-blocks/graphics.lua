local Field = {
    size_x = 10,
    size_y = 16,

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


local function draw_border()
    local border_size_x = Field.size_x * Cell.size + Field:offset() * 2
    local border_size_y = Field.size_y * Cell.size + Field:offset() * 2

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

function draw()
    draw_border()

    for i = 0, Field.size_x - 1 do
        for j = 0, Field.size_y - 1 do
            love.graphics.rectangle(
                "fill",
                Field:offset() + i * Cell.size + Block:offset(),
                Field:offset() + j * Cell.size + Block:offset(),
                Block.size, Block.size
            )
        end
    end
end
