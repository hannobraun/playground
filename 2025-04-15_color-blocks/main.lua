local lick = require "lick"

local graphics = require "graphics"


local field = {
    size_x = 10,
    size_y = 16,

    cells = {}
}

function field.index(x, y)
    return y * field.size_y + x
end

function field:get(x, y)
    return self.cells[self.index(x, y)]
end

function field:set(x, y)
    self.cells[self.index(x, y)] = true
end


function love.load()
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