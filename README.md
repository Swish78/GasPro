# GasPro

GasPro is a web application designed for managing service requests in a Gas provider company. The application allows customers to register and submit service requests, track their status, and view their account information. Staff can log in to manage service requests, provide support to customers, and track the progress of the requests.

## Features

- **Customer Features:**
  - Register and log in to the system
  - Submit service requests
  - Track the status of submitted service requests

- **Staff Features:**
  - Log in to the system
  - Manage service requests submitted by customers
  - Provide support and track service progress

## Technologies Used

- **Frontend:**
  - HTML
  - Tailwind CSS for styling

- **Backend:**
  - Django (with Django REST Framework)
  - sqlite for database management

- **Authentication:**
  - Role-Based Access Control (RBAC) for customer and staff access

## Setup Instructions

### 1. Clone the Repository

```bash
git clone https://github.com/Swish78/GasPro.git
cd GasPro
```

### 2. Install Dependencies

- For **Backend** (Django):
  ```bash
  cd gaspro
  pip install -r requirements.txt
  ```

### 4. Run Migrations

```bash
cd gaspro
python manage.py migrate
```

### 5. Run the Development Servers

- For the **Backend** (Django):
  ```bash
  cd gaspro
  python manage.py runserver
  ```


### 6. Access the Application

- **Backend**: The Django API will be available at `http://localhost:8000`.

## License

This project is open-source and available under the MIT License.