from ._rust import *
import numpy as np
import typing as T

def sum_as_string_py(a:int,b:int) -> str:
    return f"{a+b}"

def a_lot_of_sums_as_string_py(a:T.List[int],b:T.List[int]) -> T.List[str]:
    return [f"{a+b}" for a,b in zip(a,b)]

def axpy_py(a:float,x:np.ndarray,y:np.ndarray) -> np.ndarray:
    return a*x+y

def gini_py(categorical_values:np.ndarray) -> float:
    counts = np.bincount(categorical_values)
    total = np.sum(counts)
    ps = counts/total
    gini = np.sum(ps * (1.0 - ps))
    return gini