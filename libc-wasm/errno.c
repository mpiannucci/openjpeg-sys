static int __errno__ = 0;

int *__errno_location(void)
{
	return &__errno__;
}

