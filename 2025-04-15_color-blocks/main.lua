local lick = require "lick"

local graphics = require "graphics"


local field = {
    size_x = 10,
    size_y = 16,
}

function field.index(x, y)
    return y * field.size_y + x
end

function field:set(x, y)
    self[self.index(x, y)] = true
end


function love.load()
    for x = 0, field.size_x - 1 do
        for y = 0, field.size_y - 1 do
            field:set(x, y)
        end
    end

    field:set(5, 0)
end

function love.keypressed(key)
    if key == "escape" then
        love.event.quit()
    end
end

function love.draw()
    graphics.draw(field)
end