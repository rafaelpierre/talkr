from locust import HttpUser, TaskSet, between, task


def index(l):
    l.client.get("/")


class UserTasks(TaskSet):
    # one can specify tasks like this
    tasks = [index]


class WebsiteUser(HttpUser):
    """
    User class that does requests to the locust web server running on localhost
    """

    host = "http://127.0.0.1:8089"
    wait_time = between(2, 5)
    tasks = [UserTasks]