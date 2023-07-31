from geocoderpy import get_info

res = get_info(116.3883,39.9289)
print(res.iso_a2, res.name_sort)
