from django.db import models
from django.contrib.auth.models import AbstractUser, Group, Permission

# Create your models here.


class User(AbstractUser):
    is_customer = models.BooleanField(default=False)
    is_staff = models.BooleanField(default=False)
    groups = models.ManyToManyField(
        Group,
        related_name='accounts_user_set',
        blank=True
    )
    user_permissions = models.ManyToManyField(
        Permission,
        related_name='accounts_user_permissions_set',
        blank=True
    )