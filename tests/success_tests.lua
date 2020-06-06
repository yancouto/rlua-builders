local function assertFails(f, ...)
    assert(pcall(f, ...) == false)
end

return function(unit, tup, named, ce)
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

    ces = {
        ce.Unit,
        ce.Tup(nil, 42),
        ce.Named {foo = "bar"},
        ce.Composite(tup(0, "zero")),
    }
    ce.Tup(0, nil)

    return unit, t, n, ces
end