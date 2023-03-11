from dict_picker import pick_by_path


class TestSlice:
    """Test slice syntax

    Slice syntax is supported for lists. It is possible to use negative indexes
    https://github.com/sarvensis/dict-picker/issues/2

    """
    def test_get_dotted_field_with_list(self):
        event = {"get": ["dotted"]}
        dotted_field = "get.0"
        value = pick_by_path(event, dotted_field, ".")
        assert value == "dotted"

    def test_get_dotted_field_with_nested_list(self):
        event = {"get": ["dotted", ["does_not_matter", "target"]]}
        dotted_field = "get.1.1"
        value = pick_by_path(event, dotted_field, ".")
        assert value == "target"

    def test_get_dotted_field_with_list_not_found(self):
        event = {"get": ["dotted"]}
        dotted_field = "get.0.1"
        value = pick_by_path(event, dotted_field, ".")
        assert value is None

    def test_get_dotted_field_with_list_last_element(self):
        event = {"get": ["dotted", "does_not_matter", "target"]}
        dotted_field = "get.-1"
        value = pick_by_path(event, dotted_field, ".")
        assert value == "target"

    def test_get_dotted_field_with_out_of_bounds_index(self):
        event = {"get": ["dotted", "does_not_matter", "target"]}
        dotted_field = "get.3"
        value = pick_by_path(event, dotted_field, ".")
        assert value is None

    def test_get_dotted_fields_with_list_slicing(self):
        event = {"get": ["dotted", "does_not_matter", "target"]}
        dotted_field = "get.0:2"
        value = pick_by_path(event, dotted_field, ".")
        assert value == ["dotted", "does_not_matter"]

    def test_get_dotted_fields_with_list_slicing_short(self):
        event = {"get": ["dotted", "does_not_matter", "target"]}
        dotted_field = "get.:2"
        value = pick_by_path(event, dotted_field, ".")
        assert value == ["dotted", "does_not_matter"]

    def test_get_dotted_fields_reverse_order_with_slicing(self):
        event = {"get": ["dotted", "does_not_matter", "target"]}
        dotted_field = "get.::-1"
        value = pick_by_path(event, dotted_field, ".")
        assert value == ["target", "does_not_matter", "dotted"]

    def test_get_dotted_fiels_with_list_slicing_without_start(self):
        event = {"get": ["dotted", "does_not_matter", "target"]}
        dotted_field = "get.::2"
        value = pick_by_path(event, dotted_field, ".")
        assert value == ["dotted", "target"]

    def test_get_valid_fields_in_dict_with_slicing(self):
        event = {"get": ["dotted", {"name": "does_not_matter"}, {"field":"target"}]}
        dotted_field = "get.1:.field"
        value = pick_by_path(event, dotted_field, ".")
        assert value == [None, "target"]