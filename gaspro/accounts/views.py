from django.shortcuts import render
from django.contrib.auth import login
from django.shortcuts import render, redirect
from django.contrib.auth.forms import AuthenticationForm
from django.contrib import messages
from .forms import CustomUserCreationForm


# Create your views here.


def login_view(request):
    if request.method == "POST":
        form = AuthenticationForm(request, data=request.POST)
        if form.is_valid():
            user = form.get_user()
            login(request, user)
            if user.is_staff:
                return redirect('staff_dashboard')
            return redirect('customer_dashboard')
    else:
        form = AuthenticationForm()
    return render(request, 'login.html', {'form': form})


def register_view(request):
    if request.method == 'POST':
        form = CustomUserCreationForm(request.POST)
        if form.is_valid():
            user = form.save()
            user.is_customer = True
            user.is_staff = False
            user.save()
            messages.success(request, "Registration successful. You can now log in.")
            return redirect('login')
        else:
            messages.error(request, "There was an error with the form.")
    else:
        form = CustomUserCreationForm()

    return render(request, 'register.html', {'form': form})
