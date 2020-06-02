local function assertFails(f, ...)
    assert(pcall(f, ...) == false)
end

return function(unit, tup)
    assert(unit ~= nil)
    assert(type(unit) == 'userdata')

    x = tup(1, "a")
    assert(type(x) == 'userdata')
    assertFails(function() tup() end)
    assertFails(function() tup("a", "b") end)
end