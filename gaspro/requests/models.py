from django.db import models
from django.contrib.auth import get_user_model
# Create your models here.


class ServiceRequest(models.Model):
    REQUEST_CHOICES = (
        ('repair', 'Repair'),
        ('installation', 'Installation'),
        ('maintenance', 'Maintenance'),
    )

    customer = models.ForeignKey(get_user_model(), on_delete=models.CASCADE)
    request_type = models.CharField(max_length=50, choices=REQUEST_CHOICES)
    details = models.TextField()
    files = models.FileField(upload_to='requests/', null=True, blank=True)
    status = models.CharField(max_length=50, default='Pending')
    created_at = models.DateTimeField(auto_now_add=True)
    resolved_at = models.DateTimeField(null=True, blank=True)

    def __str__(self):
        return f"Request {self.id} by {self.customer.username}"
