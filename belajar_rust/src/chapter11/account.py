class Account:
    def __init__(self, name: str):
        self.name = name
        self._active = True

    def change_name(self, new_name: str):
        self.name = new_name

    def check_status(self):
        print(f"{self.name} is active: {self._active}")

class StandardAccount(Account):
    def __init__(self, name: str):
        super(StandardAccount, self).__init__(name)
        self._type = "Standard"

    def request_as_admin(self):
        print(f"{self.name} wants to be an admin")

class AdminAccount(Account):
    def __init__(self, name: str):
        super(AdminAccount, self).__init__(name)
        self._type = "Admin"

    def disable_account(self, account: StandardAccount):
        account._active = False
        print(f"{self.name} is disabling {account.name}")


if __name__ == "__main__":
    fika = StandardAccount("Fika")
    tino = AdminAccount("Tino")

    fika.check_status()
    tino.check_status()

    tino.disable_account(fika)
    fika.check_status()

