#include "test.hpp"

Complex::Complex(double x, double y)
{
    re = x;
    im = y;
}

std::unique_ptr<Complex> new_comp(double re, double im)
{
    return std::unique_ptr<Complex>(new Complex(re, im));
}

double Complex::get_re() const
{
    return re;
}

double Complex::get_im() const
{
    return im;
}