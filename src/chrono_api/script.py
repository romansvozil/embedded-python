class SettingsBuilder:
    def account_select(self, label: str): ...
    def account_multiselect(self, label: str): ...
    def text(self, label: str): ...
    def to_json(self) -> str: ...

class ScriptBase:
    async def execute(self):
        """ Internal stuff, that shouldn't be changed by user """
        await self.run()

    async def run(self):
        """ Run the script """

    def build_settings(self, settings_builder: SettingsBuilder):
        """ Build a settings, that can be displayed in website interface """

    async def get_settings(self) -> dict:
        """ Retreive the settings """