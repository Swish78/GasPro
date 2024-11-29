from django.contrib.auth.decorators import login_required
from django.shortcuts import render, redirect
from .models import ServiceRequest
from .forms import ServiceRequestForm


# Create your views here.

@login_required
def submit_request(request):
    if request.method == "POST":
        form = ServiceRequestForm(request.POST, request.FILES)
        if form.is_valid():
            form.instance.customer = request.user
            form.save()
            return redirect('customer_dashboard')
    else:
        form = ServiceRequestForm()
    return render(request, 'submit_request.html', {'form': form})


def request_tracking(request):
    requests = ServiceRequest.objects.filter(customer=request.user)
    return render(request, 'track_requests.html', {'requests': requests})
