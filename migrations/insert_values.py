import datetime
import os
import uuid

import dotenv
import models
import sqlalchemy
import sqlalchemy.orm

dotenv.load_dotenv()

db_url = (
    f"postgresql://"
    f"{os.getenv("DB_USER")}:{os.getenv("DB_PASSWORD")}"
    f"@{os.getenv("DB_CONTAINER_HOST")}:{os.getenv("DB_CONTAINER_PORT")}"
    f"/{os.getenv("DB_NAME")}"
)

with sqlalchemy.create_engine(db_url).begin() as conn:
    for count in range(100):
        conn.execute(
            sqlalchemy.insert(models.User).values(
                id=uuid.uuid4(),
                username=f"u{count}",
                name=f"User{count}",
                premium=False,
                created_at=datetime.datetime.now(),
                updated_at=datetime.datetime.now(),
            )
        )
    conn.commit()
