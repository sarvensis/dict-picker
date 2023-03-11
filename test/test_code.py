from dict_picker import pick_by_path

class TestCore:
    """Test core functionality"""

    obj = {
        "foo": "bar",
        "baz": {
            "qux": "quux",
            "quuux": "corge"
        },
        "fred": {
            "wilma": "betty",
            "barney": "pebbles"
        },
        "arr": [
            { 'id': 123 },
            { 'id': 456 },
            { 'id': 789 },
            { 'name': 'bubbles' },
        ],
    }

    def test_get_one_field_from_dict(self):
        path = "foo"
        value = pick_by_path(self.obj, path)
        assert value == "bar"

    def test_get_one_by_path_in_dict(self):
        path = "baz/qux"
        value = pick_by_path(self.obj, path)
        assert value == "quux"

        path = "fred/wilma"
        value = pick_by_path(self.obj, path)
        assert value == "betty"

    def test_get_one_by_path_with_wildcard_in_dict(self):
        path = "*/qux"
        value = pick_by_path(self.obj, path)
        assert value == "quux"


    def test_get_dict_by_path_with_wildcard_in_dict(self):
        path = "fred/*"
        value = pick_by_path(self.obj, path)
        assert value == {"wilma": "betty", "barney": "pebbles"}

    def test_get_none_by_path_with_wildcard_in_dict(self):
        path = "*/quux"
        value = pick_by_path(self.obj, path)
        assert value is None

    def test_get_list_by_path_with_wildcard_in_dict(self):
        path = "arr/*/id"
        value = pick_by_path(self.obj, path)
        assert value == [123, 456, 789]