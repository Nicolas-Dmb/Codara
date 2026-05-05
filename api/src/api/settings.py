from pydantic_settings import BaseSettings,SettingsConfigDict


class Settings(BaseSettings):
    db_user: str
    db_password: str
    db_name: str
    db_host: str
    db_port: int
    cors_origins: list[str]

    model_config = SettingsConfigDict(env_file=".env")


settings = Settings()  # type: ignore