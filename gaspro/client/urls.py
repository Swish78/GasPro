from django.urls import path
from . import views

urlpatterns = [
    path('customer/', views.customer_dashboard, name='customer_dashboard'),
    path('staff/', views.staff_dashboard, name='staff_dashboard'),
]
