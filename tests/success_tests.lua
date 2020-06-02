local function assertFails(f, ...)
    assert(pcall(f, ...) == false)
end

return function(unit, tup, named)
    assert(unit ~= nil)
    assert(type(unit) == 'userdata')

    t = tup(1, "a")
    assert(type(t) == 'userdata')
    assertFails(function() tup() end)
    assertFails(function() tup("a", "b") end)

    n = named {a = 1, b = "a"}
    assert(type(n) == 'userdata')
    assertFails(function() named() end)
    assertFails(function() named{} end)
    assertFails(function() named{a = "asd", b = "asd"} end)

    return unit, t, n
end