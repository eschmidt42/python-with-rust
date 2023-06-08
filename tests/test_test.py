import python_package as pp
import python_with_rust as pr

def test_sum_as_string_py():
    a = 1
    b = 2

    res = pp.sum_as_string_py(a,b)
    assert res == "3"

def test_sum_as_string_rs():
    a = 1
    b = 2

    res = pr.sum_as_string_rs(a,b)
    assert res == "3"