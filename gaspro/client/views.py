from django.shortcuts import render
from django.contrib.auth.decorators import login_required
from django.apps import apps
from django.shortcuts import render, get_object_or_404, redirect
from django.core.paginator import Paginator
Services = apps.get_model('requests', 'ServiceRequest')

# Create your views here.


@login_required
def customer_dashboard(request):
    # Fetching service requests for the logged-in customer(filter)
    customer_requests = Services.objects.select_related('customer').filter(customer=request.user).order_by('-created_at')
    paginator = Paginator(customer_requests, 10)  # Paginate 10 requests per page
    page_number = request.GET.get('page')
    page_requests = paginator.get_page(page_number)

    return render(request, 'customer_dashboard.html', {'requests': page_requests})


@login_required
def staff_dashboard(request):

    requests = Services.objects.all().order_by('-created_at')
    paginator = Paginator(requests, 10)  # Paginate 10 requests per page
    page_number = request.GET.get('page')
    page_requests = paginator.get_page(page_number)

    if request.method == 'POST':
        request_id = request.POST.get('request_id')
        new_status = request.POST.get('status')
        service_request = get_object_or_404(Services, id=request_id)
        service_request.status = new_status
        if new_status == 'Resolved':
            from django.utils.timezone import now
            service_request.resolved_at = now()
        service_request.save()
        return redirect('staff_dashboard')

    return render(request, 'staff_dashboard.html', {'requests': page_requests})