from cpython cimport bool
from cpython cimport array
import array

cdef array.array span_from_str(str span_spec):
    cdef array.array span = array.array("H")
    for token in span_spec.split():
        try:
            span.append(int(token))
            continue
        except ValueError:
            pass
        (lower_bound, upper_bound) = token.split("-")
        span.extend(range(int(lower_bound), int(upper_bound) + 1))
    return span

cdef array.array _parse_span(span_spec, int num_songs):
    if isinstance(span_spec, int):
        return array.array("H", [span_spec,])
    elif isinstance(span_spec, str):
        if span_spec == "*":
            return span_from_str("1-{}".format(num_songs))
        return span_from_str(span_spec)
    return array.array("H")

cdef bool span_is_well_formed(array.array span, int num_songs):
    for value in span:
        if value > num_songs or value < 1:
            return False
    return len(span) > 0

def parse_span(span_spec, int num_songs):
    parsed = _parse_span(span_spec, num_songs)
    if not span_is_well_formed(parsed, num_songs):
        raise ValueError("bad span: {}".format(span_spec))
    return parsed

cdef assert_tag_values_are_strings(list tag_values):
    for value in tag_values:
        if not isinstance(value, str):
            raise ValueError(
                "unexpected non-str detected: ``{}''".format(str(tag_values)))

def listify_tag_values(raw_tag_values):
    if (isinstance(raw_tag_values, list)):
        assert_tag_values_are_strings(raw_tag_values)
        return list(raw_tag_values)
    elif (isinstance(raw_tag_values, dict)):
        raise ValueError(
                "unexpected dict value {}".format(str(raw_tag_values)))
    return [str(raw_tag_values),]
