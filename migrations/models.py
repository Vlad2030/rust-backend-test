import datetime
import uuid

import sqlalchemy
import sqlalchemy.ext.declarative
import sqlalchemy.orm

Base = sqlalchemy.ext.declarative.declarative_base()
metadata = sqlalchemy.MetaData()


class User(Base):
    __tablename__ = "users"

    id = sqlalchemy.orm.mapped_column(
        sqlalchemy.UUID,
        primary_key=True,
        unique=True,
        nullable=False,
        default=uuid.uuid4,
    )
    username = sqlalchemy.orm.mapped_column(
        sqlalchemy.VARCHAR,
        primary_key=False,
        unique=True,
        nullable=True,
        index=True,
        default=None,
    )
    name = sqlalchemy.orm.mapped_column(
        sqlalchemy.VARCHAR,
        primary_key=False,
        unique=False,
        nullable=False,
        index=False,
        default="User",
    )
    premium = sqlalchemy.orm.mapped_column(
        sqlalchemy.BOOLEAN,
        primary_key=False,
        unique=False,
        nullable=False,
        index=True,
        default=False,
    )
    created_at = sqlalchemy.orm.mapped_column(
        sqlalchemy.TIMESTAMP(timezone=True),
        primary_key=False,
        unique=False,
        nullable=False,
        index=False,
        default=datetime.datetime.now,
    )
    updated_at = sqlalchemy.orm.mapped_column(
        sqlalchemy.TIMESTAMP(timezone=True),
        primary_key=False,
        unique=False,
        nullable=False,
        index=False,
        default=datetime.datetime.now,
    )
