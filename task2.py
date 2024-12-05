# The engineers are trying to figure out which reports are safe. The Red-Nosed reactor safety systems can only tolerate levels that are either gradually increasing or gradually decreasing. So, a report only counts as safe if both of the following are true:

# The levels are either all increasing or all decreasing.
# Any two adjacent levels differ by at least one and at most three.

reports = []
with open('input_task_2.csv', 'r', newline='') as file:
    for row in file:
        reports.append([int(x) for x in row.split()])

unsafe_reports = 0
part = 2

if part == 1:
    def check_safe(report, decreasing: bool):
        d_list = []
        acs = []
        for i in range(0, len(report) - 1):
            if decreasing:
                if report[i] < report[i + 1]:
                    print(report, "Decreasing: ", report[i], report[i + 1])
                    return False
            else:
                if report[i] > report[i + 1]:
                    print(report, "Increasing: ", report[i], report[i + 1])
                    return False
            difference = abs(report[i] - report[i + 1])
            d_list.append(difference)
            if report[i] < report[i + 1]:
                acs.append(True)
            else:
                acs.append(False)
            if difference > 3 or difference < 1:
                print(report, "Difference: ", difference)
                return False
        # print(report, d_list, acs)
        return True

    print(len(reports))
    for r in reports:
        if len(r) < 2:
            continue
        if r[0] > r[1]:
            if not check_safe(r, True):
                unsafe_reports += 1
        else:
            if not check_safe(r, False):
                unsafe_reports += 1

    print("Safe reports: ", len(reports)-unsafe_reports)

if part == 2:
    safe_reports = 0
    def check_safe(report, level_removed: bool = False):
        print(level_removed, report)
        decreasing = True
        if report[0] < report[1]:
            decreasing = False
        for i in range(0, len(report) - 1):
            if decreasing:
                if report[i] < report[i + 1]:
                    # if not level_removed:
                        # return check_safe(report[:i+1] + (report[i + 2:] if len(report) > i + 2 else []), True)
                    return False
            else:
                if report[i] > report[i + 1]:
                    # if not level_removed:
                        # return check_safe(report[:i+1] + (report[i + 2:] if len(report) > i + 2 else []), True)
                    return False
            difference = abs(report[i] - report[i + 1])
            if difference > 3 or difference < 1:
                # if not level_removed:
                    # return check_safe(report[:i+1] + (report[i + 2:] if len(report) > i + 2 else []), True)
                return False
        return True

    for r in reports:
        if len(r) < 2:
            continue
        if not check_safe(r):
            for i in range(0, len(r)):
                if check_safe(r[:i] + (r[i+1:] if len(r) - 1 != i else [])):
                    safe_reports += 1
                    break
        else:
            safe_reports += 1

    print("Safe reports: ", safe_reports)




