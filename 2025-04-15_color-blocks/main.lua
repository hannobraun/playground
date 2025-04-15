local lick = require "lick"

local graphics = require "graphics"


local Field = {}

function Field.new()
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

    return field
end

local field = Field.new()


function love.load()
    field:set(5, 0)
end

function love.keypressed(key)
    if key == "escape" then
        love.event.quit()
    end
end

function love.update()
    local next = Field.new()

    for x = 0, field.size_x - 1 do
        for y = 0, field.size_y - 1 do
            if field:get(x, y) then
                next:set(x, y + 1)
            end
        end
    end

    field = next
end

function love.draw()
    graphics.draw(field)
end