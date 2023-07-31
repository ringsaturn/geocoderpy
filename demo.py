from geocoderpy import get_info

res = get_info(116.3883, 39.9289)
print(res.name, res.code)
