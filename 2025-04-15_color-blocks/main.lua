local lick = require "lick"

local graphics = require "graphics"


local field = {
    size_x = 10,
    size_y = 16,
}


function love.keypressed(key)
    if key == "escape" then
        love.event.quit()
    end
end

function love.draw()
    graphics.draw(field)
end