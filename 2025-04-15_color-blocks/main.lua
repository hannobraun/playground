local lick = require "lick"

local graphics = require "graphics"


function love.keypressed(key)
    if key == "escape" then
        love.event.quit()
    end
end

function love.draw()
    graphics.draw()
end