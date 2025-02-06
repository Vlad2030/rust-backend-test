import os
import string
import uuid
import random
import http

import dotenv
import requests

dotenv.load_dotenv()

backend_host = os.getenv("BACKEND_HOST")
backend_port = os.getenv("BACKEND_PORT")

backend_url = f"http://{backend_host}:{backend_port if backend_port else 80}"


def test_get_healthcheck() -> None:
    healthcheck = requests.get(backend_url + "/health/")
    assert healthcheck.status_code == http.HTTPStatus.NOT_FOUND

    healthcheck = requests.get(backend_url + "/health")
    assert healthcheck.status_code == http.HTTPStatus.OK
    assert healthcheck.json()["successful"] == True


def test_get_users() -> None:
    users = requests.get(
        backend_url + "/users/",
        params={"limit": 100, "offset": 0},
    )
    assert users.status_code == http.HTTPStatus.NOT_FOUND

    users = requests.get(
        backend_url + "/users",
        params={"limit": 100, "offset": 0},
    )
    assert users.status_code == http.HTTPStatus.OK


def test_get_users_length() -> None:
    users = requests.get(
        backend_url + "/users",
        params={"limit": 10, "offset": 10},
    )
    user_json = users.json()
    assert user_json["count"] == 10
    assert user_json["limit"] == 10
    assert user_json["offset"] == 10
    assert len(user_json["users"]) == 10


def test_get_users_with_default_params() -> None:
    users = requests.get(backend_url + "/users")
    assert users.status_code == http.HTTPStatus.OK


def test_get_users_with_wrong_param_type() -> None:
    users = requests.get(
        backend_url + "/users",
        params={"limit": "100s", "offset": 0},
    )
    assert users.status_code == http.HTTPStatus.INTERNAL_SERVER_ERROR


def test_get_users_with_wrong_params() -> None:
    users = requests.get(
        backend_url + "/users",
        params={"limit": 1000},
    )
    assert users.status_code == http.HTTPStatus.BAD_REQUEST

    users = requests.get(
        backend_url + "/users",
        params={"offset": -1},
    )
    assert users.status_code == http.HTTPStatus.BAD_REQUEST


def test_get_users_by_id() -> None:
    users = requests.get(
        backend_url + "/users",
        params={"limit": 1, "offset": 0},
    )
    users_json = users.json()

    user = requests.get(
        backend_url + "/users",
        params={
            "id": users_json["users"][0]["id"],
        },
    )
    assert user.status_code == http.HTTPStatus.OK


def test_get_users_by_id_empty() -> None:
    user = requests.get(
        backend_url + "/users",
        params={"id": ""},
    )
    assert user.status_code == http.HTTPStatus.INTERNAL_SERVER_ERROR


def test_get_users_by_id_random() -> None:
    user = requests.get(
        backend_url + "/users",
        params={
            "id": str(uuid.uuid4()),
        },
    )
    user_json = user.json()
    assert len(user_json["users"]) == 0
    assert user.status_code == http.HTTPStatus.OK


def test_get_users_by_username() -> None:
    users = requests.get(
        backend_url + "/users",
        params={"limit": 1, "offset": 0},
    )
    users_json = users.json()

    user = requests.get(
        backend_url + "/users",
        params={
            "username": users_json["users"][0]["username"],
        },
    )
    assert user.status_code == http.HTTPStatus.OK


def test_get_users_by_username_empty() -> None:
    user = requests.get(
        backend_url + "/users",
        params={
            "username": "",
        },
    )
    user_json = user.json()
    assert len(user_json["users"]) == 0
    assert user.status_code == http.HTTPStatus.OK


def test_get_users_by_username_random() -> None:
    user = requests.get(
        backend_url + "/users",
        params={
            "username": "".join([string.ascii_letters[_] for _ in range(10)]),
        },
    )
    user_json = user.json()
    assert len(user_json["users"]) == 0
    assert user.status_code == http.HTTPStatus.OK


def test_post_user() -> None:
    user = requests.post(backend_url + "/users/")
    assert user.status_code == http.HTTPStatus.NOT_FOUND

    user = requests.post(
        backend_url + "/users",
        params={
            "username": "".join(
                [random.choice(list(string.ascii_letters)) for _ in range(10)]
            ),
        },
    )
    assert user.status_code == http.HTTPStatus.CREATED


def test_post_user_with_empty_params() -> None:
    user = requests.post(backend_url + "/users")
    assert user.status_code == http.HTTPStatus.CREATED


def test_post_create_user_with_busy_username() -> None:
    users = requests.get(
        backend_url + "/users",
        params={"limit": 100, "offset": 0},
    )
    busy_username = users.json()["users"][0]["username"]

    user = requests.post(
        backend_url + "/users",
        params={"username": busy_username},
    )
    assert user.status_code == http.HTTPStatus.BAD_REQUEST


def test_delete_user() -> None:
    users = requests.delete(backend_url + "/users/")
    assert users.status_code == http.HTTPStatus.NOT_FOUND

    users = requests.get(
        backend_url + "/users",
        params={"limit": 100, "offset": 0},
    )
    user_id = users.json()["users"][-1]["id"]

    users = requests.delete(backend_url + "/users", params={"id": user_id})
    assert users.status_code == http.HTTPStatus.OK


def test_delete_user_with_empty_params() -> None:
    user = requests.delete(
        backend_url + "/users",
        params={},
    )
    assert user.status_code == http.HTTPStatus.BAD_REQUEST


def test_delete_user_with_fake_id() -> None:
    user = requests.delete(
        backend_url + "/users",
        params={"id": uuid.uuid4()},
    )
    assert user.status_code == http.HTTPStatus.NOT_FOUND


def test_put_user() -> None:
    user = requests.put(
        backend_url + "/users/",
    )
    assert user.status_code == http.HTTPStatus.NOT_FOUND

    user = requests.put(
        backend_url + "/users",
    )
    assert user.status_code == http.HTTPStatus.BAD_REQUEST


def test_put_user_with_empty_params() -> None:
    user = requests.put(
        backend_url + "/users",
        params={"id": ""},
    )
    assert user.status_code == http.HTTPStatus.INTERNAL_SERVER_ERROR


def test_put_real_user() -> None:
    user = requests.get(
        backend_url + "/users",
    )
    user_json = user.json()

    user = requests.put(
        backend_url + "/users",
        params={
            "id": user_json["users"][-1]["id"],
            "name": "".join(
                [random.choice(list(string.ascii_letters)) for _ in range(20)]
            ),
        },
    )

    assert user.status_code == http.HTTPStatus.OK


def test_put_real_user_with_busy_username() -> None:
    user = requests.get(
        backend_url + "/users",
    )
    user_json = user.json()

    user = requests.put(
        backend_url + "/users",
        params={
            "id": user_json["users"][0]["id"],
            "username": user_json["users"][-1]["username"],
        },
    )

    assert user.status_code == http.HTTPStatus.BAD_REQUEST

    # users = requests.get(
    #     backend_url + "/users",
    #     params={"limit": 10, "offset": 10},
    # )
    # user_json = users.json()
    # assert user_json["count"] == 10
